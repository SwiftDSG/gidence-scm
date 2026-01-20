use std::{collections::BTreeMap, fs::read_to_string, rc::Rc, str::FromStr};

use actix_service::{Service, Transform};
use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    web, Error, HttpMessage,
};
use chrono::Utc;
use futures::{
    future::{ready, LocalBoxFuture, Ready},
    FutureExt, StreamExt,
};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use mongodb::{
    bson::{doc, from_document, oid::ObjectId, to_bson},
    Database,
};
use pwhash::bcrypt;
use serde::{Deserialize, Serialize};

use super::event::EventKind;

const COLLECTION: &str = "users";

static mut KEYS: BTreeMap<String, String> = BTreeMap::new();

#[derive(Deserialize)]
pub struct UserRefreshRequest {
    pub rtk: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct UserRequest {
    pub cluster_id: Vec<ObjectId>,
    pub number: String,
    pub name: String,
    pub password: String,
    pub role: UserRole,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub _id: ObjectId,
    pub cluster_id: Vec<ObjectId>,
    pub number: String,
    pub name: String,
    pub password: String,
    pub role: UserRole,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct UserResponse {
    pub id: String,
    pub cluster_id: Vec<String>,
    pub number: String,
    pub name: String,
    pub role: UserRole,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct UserMinimalResponse {
    pub id: String,
    pub cluster_id: Vec<String>,
    pub number: String,
    pub name: String,
    pub role: UserRole,
}
#[derive(PartialEq, Eq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UserRole {
    SuperAdmin,
    Manager,
    Officer,
}

#[derive(Debug, Serialize, Deserialize)]
struct UserClaim {
    aud: String,
    exp: i64,
    iss: String,
    sub: String,
}
#[derive(Debug)]
pub struct UserAuthenticationData {
    pub _id: ObjectId,
    pub role: UserRole,
    pub token: String,
}
#[derive(Debug, Deserialize)]
pub struct UserCredential {
    pub number: String,
    pub password: String,
}
pub struct UserAuthenticationMiddleware<S> {
    service: Rc<S>,
}
pub struct UserAuthenticationMiddlewareFactory;

pub type UserAuthentication = Rc<UserAuthenticationData>;

#[derive(Debug, Deserialize)]
pub struct UserQuery {
    pub cluster_id: Option<ObjectId>,
    pub cluster_eid: Option<ObjectId>,
    pub text: Option<String>,
    pub limit: Option<usize>,
    pub skip: Option<usize>,
}

impl From<UserRequest> for User {
    fn from(a: UserRequest) -> Self {
        Self {
            _id: ObjectId::new(),
            cluster_id: a.cluster_id,
            number: a.number,
            password: a.password,
            name: a.name,
            role: a.role,
        }
    }
}
impl From<User> for UserResponse {
    fn from(a: User) -> Self {
        Self {
            id: a._id.to_string(),
            cluster_id: a
                .cluster_id
                .clone()
                .iter_mut()
                .map(|a| a.to_string())
                .collect(),
            number: a.number,
            name: a.name,
            role: a.role,
        }
    }
}

impl User {
    pub async fn save(&mut self, db: &Database) -> Result<(), EventKind> {
        let collection = db.collection::<Self>(COLLECTION);

        if let Ok(hash) = bcrypt::hash(&self.password) {
            self.password = hash;
            if collection.insert_one(self, None).await.is_ok() {
                Ok(())
            } else {
                Err(EventKind::SavingFailed)
            }
        } else {
            Err(EventKind::SavingFailed)
        }
    }
    pub async fn update(
        &mut self,
        password: Option<String>,
        db: &Database,
    ) -> Result<(), EventKind> {
        let collection = db.collection::<Self>(COLLECTION);

        if let Some(password) = password {
            if let Ok(hash) = bcrypt::hash(password) {
                self.password = hash;
            }
        }

        if collection
            .update_one(
                doc! { "_id": self._id },
                doc! { "$set": to_bson::<Self>(self).unwrap() },
                None,
            )
            .await
            .is_ok()
        {
            Ok(())
        } else {
            Err(EventKind::UpdatingFailed)
        }
    }
    pub async fn delete(&self, db: &Database) {
        let collection = db.collection::<Self>(COLLECTION);

        let _ = collection.delete_one(doc! { "_id": self._id }, None).await;
    }
    pub async fn find_by_id(_id: &ObjectId, db: &Database) -> Result<Self, EventKind> {
        let collection = db.collection::<Self>(COLLECTION);

        if let Ok(Some(user)) = collection.find_one(doc! { "_id": _id }, None).await {
            Ok(user)
        } else {
            Err(EventKind::SavingFailed)
        }
    }
    pub async fn find_by_number(number: &String, db: &Database) -> Result<Self, EventKind> {
        let collection = db.collection::<Self>(COLLECTION);

        if let Ok(Some(user)) = collection.find_one(doc! { "number": number }, None).await {
            Ok(user)
        } else {
            Err(EventKind::SavingFailed)
        }
    }
    pub async fn find_all(db: &Database) -> Result<Vec<Self>, EventKind> {
        let collection = db.collection::<Self>(COLLECTION);

        match collection.find(doc! {}, None).await {
            Ok(mut cursor) => {
                let mut users = Vec::new();
                while let Some(Ok(user)) = cursor.next().await {
                    users.push(user);
                }

                if users.is_empty() {
                    Err(EventKind::NotFound)
                } else {
                    Ok(users)
                }
            }
            Err(e) => {
                println!("ERROR: {:?}", e);
                Err(EventKind::FindingFailed)
            }
        }
    }
    pub async fn find_many_by_cluster_id(
        cluster_id: &ObjectId,
        db: &Database,
    ) -> Result<Vec<Self>, EventKind> {
        let collection = db.collection::<Self>(COLLECTION);

        let mut pipeline = Vec::new();
        let mut queries = Vec::new();

        queries.push(doc! {
            "$in": [to_bson::<ObjectId>(cluster_id).unwrap(), "$cluster_id"]
        });
        queries.push(doc! {
            "$eq": [ "$role", "super_admin" ]
        });

        pipeline.push(doc! {
            "$match": {
                "$expr": {
                    "$or": queries
                }
            }
        });

        match collection.aggregate(pipeline, None).await {
            Ok(mut cursor) => {
                let mut users = Vec::new();
                while let Some(Ok(doc)) = cursor.next().await {
                    let user = from_document::<Self>(doc).unwrap();
                    users.push(user);
                }
                if !users.is_empty() {
                    Ok(users)
                } else {
                    Err(EventKind::NotFound)
                }
            }
            Err(e) => {
                println!("ERROR: {:?}", e);
                Err(EventKind::FindingFailed)
            }
        }
    }
    pub async fn find_many_minimal(
        query: &UserQuery,
        db: &Database,
    ) -> Result<Vec<UserMinimalResponse>, EventKind> {
        let collection = db.collection::<Self>(COLLECTION);

        let mut pipeline = Vec::new();
        let mut queries = Vec::new();

        if let Some(text) = &query.text {
            queries.push(doc! {
                "$regexMatch": {
                    "input": "$name",
                    "options": "i",
                    "regex": to_bson::<String>(text).unwrap()
                }
            });
        }
        if let Some(cluster_id) = &query.cluster_id {
            queries.push(doc! {
                "$in": [to_bson::<ObjectId>(cluster_id).unwrap(), "$cluster_id"]
            });
        }
        if let Some(cluster_id) = &query.cluster_eid {
            queries.push(doc! {
                "$eq": [
                    {
                        "$in": [to_bson::<ObjectId>(cluster_id).unwrap(), "$cluster_id"]
                    },
                    false
                ]
            });
            queries.push(doc! {
                "$ne": [ "$role", "super_admin" ]
            });
        }

        pipeline.push(doc! {
            "$match": {
                "$expr": {
                    "$and": queries
                }
            }
        });
        pipeline.push(doc! {
            "$project": {
                "id": {
                    "$toString": "$_id"
                },
                "cluster_id": {
                    "$map": {
                        "input": "$cluster_id",
                        "in": { "$toString": "$$this" }
                    }
                },
                "name": "$name",
                "number": "$number",
                "role": "$role",
            }
        });

        if let Some(skip) = query.skip {
            pipeline.push(doc! {
                "$skip": to_bson::<usize>(&skip).unwrap()
            });
        }
        if let Some(limit) = query.limit {
            pipeline.push(doc! {
                "$limit": to_bson::<usize>(&limit).unwrap()
            });
        }

        match collection.aggregate(pipeline, None).await {
            Ok(mut cursor) => {
                let mut users = Vec::new();
                while let Some(Ok(doc)) = cursor.next().await {
                    let user = from_document::<UserMinimalResponse>(doc).unwrap();
                    users.push(user);
                }
                if !users.is_empty() {
                    Ok(users)
                } else {
                    Err(EventKind::NotFound)
                }
            }
            Err(e) => {
                println!("ERROR: {:?}", e);
                Err(EventKind::FindingFailed)
            }
        }
    }
    pub async fn super_admin_available(db: &Database) -> bool {
        let collection = db.collection::<Self>(COLLECTION);

        matches!(
            collection
                .find_one(
                    doc! { "role": to_bson::<UserRole>(&UserRole::SuperAdmin).unwrap() },
                    None,
                )
                .await,
            Ok(Some(_))
        )
    }
}

impl UserCredential {
    pub async fn authenticate(
        &self,
        db: &Database,
    ) -> Result<((String, String), UserResponse), EventKind> {
        let user = match User::find_by_number(&self.number, db).await {
            Ok(user) => user,
            Err(_) => return Err(EventKind::NotFound),
        };
        if !bcrypt::verify(self.password.clone(), &user.password) {
            return Err(EventKind::InvalidCombination);
        }

        let claim_access = UserClaim {
            sub: user._id.to_string(),
            exp: Utc::now().timestamp() + 1800,
            iss: "Redian".to_string(),
            aud: std::env::var("BASE_URL").unwrap(),
        };
        let claim_refresh = UserClaim {
            sub: user._id.to_string(),
            exp: Utc::now().timestamp() + 259200,
            iss: "Redian".to_string(),
            aud: std::env::var("BASE_URL").unwrap(),
        };

        let header = Header::new(Algorithm::RS256);
        unsafe {
            match (
                encode(
                    &header,
                    &claim_access,
                    &EncodingKey::from_rsa_pem(KEYS.get("private_access").unwrap().as_bytes())
                        .unwrap(),
                ),
                encode(
                    &header,
                    &claim_refresh,
                    &EncodingKey::from_rsa_pem(KEYS.get("private_refresh").unwrap().as_bytes())
                        .unwrap(),
                ),
            ) {
                (Ok(atk), Ok(rtk)) => match User::find_by_id(&user._id, db).await {
                    Ok(user) => Ok(((atk, rtk), UserResponse::from(user))),
                    _ => Err(EventKind::NotFound),
                },
                _ => Err(EventKind::InvalidCombination),
            }
        }
    }
    pub async fn refresh(
        token: &str,
        db: &Database,
    ) -> Result<(String, String, UserResponse), EventKind> {
        let validation = Validation::new(Algorithm::RS256);
        let data;

        unsafe {
            let key = KEYS.get("public_refresh").unwrap();

            data = decode::<UserClaim>(
                token,
                &DecodingKey::from_rsa_pem(key.as_bytes()).unwrap(),
                &validation,
            )
            .map_err(|e| {
                println!("ERROR: {}", e.to_string());
                EventKind::InvalidToken
            })?;
        }
        let _id = ObjectId::from_str(&data.claims.sub).map_err(|_| EventKind::InvalidId)?;

        let user = match User::find_by_id(&_id, db).await {
            Ok(user) => user,
            _ => return Err(EventKind::NotFound),
        };

        let claim_access = UserClaim {
            sub: user._id.to_string(),
            exp: Utc::now().timestamp() + 1800,
            iss: "Redian".to_string(),
            aud: std::env::var("BASE_URL").unwrap(),
        };
        let claim_refresh = UserClaim {
            sub: user._id.to_string(),
            exp: Utc::now().timestamp() + 259200,
            iss: "Redian".to_string(),
            aud: std::env::var("BASE_URL").unwrap(),
        };

        let header = Header::new(Algorithm::RS256);
        unsafe {
            match (
                encode(
                    &header,
                    &claim_access,
                    &EncodingKey::from_rsa_pem(KEYS.get("private_access").unwrap().as_bytes())
                        .unwrap(),
                ),
                encode(
                    &header,
                    &claim_refresh,
                    &EncodingKey::from_rsa_pem(KEYS.get("private_refresh").unwrap().as_bytes())
                        .unwrap(),
                ),
            ) {
                (Ok(atk), Ok(rtk)) => match User::find_by_id(&user._id, db).await {
                    Ok(user) => Ok((atk, rtk, UserResponse::from(user))),
                    _ => Err(EventKind::NotFound),
                },
                _ => Err(EventKind::NotFound),
            }
        }
    }
    pub fn verify(token: &str) -> Option<ObjectId> {
        let validation: Validation = Validation::new(Algorithm::RS256);
        unsafe {
            match decode::<UserClaim>(
                token,
                &DecodingKey::from_rsa_pem(KEYS.get("public_access").unwrap().as_bytes()).unwrap(),
                &validation,
            ) {
                Ok(data) => match ObjectId::from_str(&data.claims.sub) {
                    Ok(id) => Some(id),
                    Err(_) => None,
                },
                Err(_) => None,
            }
        }
    }
}

impl<S, B> Service<ServiceRequest> for UserAuthenticationMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    actix_service::forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let srv = self.service.clone();

        async move {
            if let Some(db) = req
                .app_data::<web::Data<Database>>()
                .map(|data| data.get_ref())
            {
                let headers: &actix_web::http::header::HeaderMap = req.headers();
                if let Some(bearer_token) = headers.get("Authorization") {
                    let mut bytes_token = Vec::new();
                    for i in bearer_token.as_bytes() {
                        bytes_token.push(*i);
                    }
                    if bytes_token.len() > 7 {
                        bytes_token.drain(0..7);
                        let token = String::from_utf8(bytes_token).unwrap();
                        if let Some(_id) = UserCredential::verify(&token) {
                            if let Ok(user) = User::find_by_id(&_id, db).await {
                                let auth_data = UserAuthenticationData {
                                    _id,
                                    role: user.role,
                                    token,
                                };
                                req.extensions_mut()
                                    .insert::<UserAuthentication>(Rc::new(auth_data));
                            }
                        }
                    }
                }
            }
            let res = srv.call(req).await?;
            Ok(res)
        }
        .boxed_local()
    }
}
impl<S, B> Transform<S, ServiceRequest> for UserAuthenticationMiddlewareFactory
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = UserAuthenticationMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(UserAuthenticationMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub fn load_keys() {
    let private_access_file =
        read_to_string("./keys/private_access.key").expect("LOAD_FAILED_PRIVATE_ACCESS");
    let public_access_file =
        read_to_string("./keys/public_access.pem").expect("LOAD_FAILED_PUBLIC_ACCESS");
    let private_refresh_file =
        read_to_string("./keys/private_refresh.key").expect("LOAD_FAILED_PRIVATE_ACCESS");
    let public_refresh_file =
        read_to_string("./keys/public_refresh.pem").expect("LOAD_FAILED_PUBLIC_ACCESS");
    unsafe {
        KEYS.insert("private_access".to_string(), private_access_file);
        KEYS.insert("public_access".to_string(), public_access_file);
        KEYS.insert("private_refresh".to_string(), private_refresh_file);
        KEYS.insert("public_refresh".to_string(), public_refresh_file);
    }
}
