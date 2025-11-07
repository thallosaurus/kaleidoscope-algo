from PIL import Image
import numpy as np
import cv2

def score_image(path):
    img = np.array(Image.open(path).convert("RGB"))
    hsv = cv2.cvtColor(img, cv2.COLOR_RGB2HSV)

    # 1. Kontrast
    contrast = np.std(cv2.cvtColor(img, cv2.COLOR_RGB2GRAY))

    # 2. Sättigung
    saturation = np.mean(hsv[..., 1])

    # 3. Kantenenergie
    edges = cv2.Canny(cv2.cvtColor(img, cv2.COLOR_RGB2GRAY), 100, 200)
    edge_energy = np.sum(edges) / edges.size

    # 4. Symmetrie
    h, w, _ = img.shape
    left = img[:, :w//2, :]
    right = np.flip(img[:, w//2:, :], axis=1)
    symmetry = 1 - np.mean(np.abs(left - right)) / 255.0

    # Gesamtwert (gewichtete Mischung)
    return 0.4*contrast + 0.2*saturation + 0.3*edge_energy + 0.1*symmetry