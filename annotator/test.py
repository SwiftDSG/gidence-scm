from ultralytics import YOLO

model = YOLO('./annotator/best.pt')

# Test on 5 validation images
results = model.predict(
    source='./annotator/images',
    conf=0.25,
    save=True,
    max_det=50
)