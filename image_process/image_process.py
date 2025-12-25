import numpy as np
import cv2
import sys

IMAGE_FILE = "input.png"
POINTS_TO_SAMPLE = 15

def save_points(filename, values):
    with open(filename, 'w') as f:
        for i, val in enumerate(values):
            f.write(f"{float(i)} {float(val)}\n")


def main():
    img = cv2.imread(IMAGE_FILE, 0)
    if img is None:
        print(f"Error: Could not read {IMAGE_FILE}")
        sys.exit(1)

    contours, _ = cv2.findContours(img, cv2.RETR_EXTERNAL, cv2.CHAIN_APPROX_SIMPLE)

    if not contours:
        print("No shape found in image!")
        sys.exit(1)

    contour = max(contours, key=cv2.contourArea).reshape(-1, 2)

    step = 1
    if step == 0: step = 1

    sampled_points = contour[::step]

    sampled_points = np.vstack([sampled_points, sampled_points[0]])

    x_coords = sampled_points[:, 0]
    y_coords = sampled_points[:, 1]
    
    height = img.shape[0]
    y_coords = height - y_coords 

    save_points("points_x.txt", x_coords)
    save_points("points_y.txt", y_coords)
    
    print(f"Successfully processed image. Saved {len(sampled_points)} points.")

if __name__ == "__main__":
    main()
