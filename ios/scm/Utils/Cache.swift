//
//  Cache.swift
//  scm
//
//  Created by Kemal Dwi Heldy Muhammad on 20/10/24.
//

import SwiftUI

class ImageCache {
    static let shared = ImageCache()
    
    private var cache = NSCache<NSString, UIImage>()
    
    func image(for key: String) -> UIImage? {
        self.cache.object(forKey: NSString(string: key))
    }
    
    func setImage(_ image: UIImage, for key: String) {
        self.cache.setObject(image, forKey: NSString(string: key))
    }
}
