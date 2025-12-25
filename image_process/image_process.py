import cv2
import numpy as np
import sys

# CONFIGURATION
IMAGE_FILE = "input.png" # You need an image file here!
POINTS_TO_SAMPLE = 15    # Keep this LOW (10-20) to prevent glitchy spikes

def save_points(filename, values):
    with open(filename, 'w') as f:
        # We use 'i' as time (0.0, 1.0, 2.0...)
        for i, val in enumerate(values):
            f.write(f"{float(i)} {float(val)}\n")

def main():
    # 1. Load Image
    img = cv2.imread(IMAGE_FILE, 0) # Load as grayscale
    if img is None:
        print(f"Error: Could not read {IMAGE_FILE}")
        sys.exit(1)

    # 2. Find Contours (the outline)
    contours, _ = cv2.findContours(img, cv2.RETR_EXTERNAL, cv2.CHAIN_APPROX_SIMPLE)
    
    if not contours:
        print("No shape found in image!")
        sys.exit(1)
        
    # Take the largest contour
    contour = max(contours, key=cv2.contourArea).reshape(-1, 2)

    # 3. Downsample (Pick only a few points)
    # If we take every single pixel, the polynomial will go crazy.
    step = len(contour) // POINTS_TO_SAMPLE
    if step == 0: step = 1
    
    sampled_points = contour[::step]
    
    # Close the loop (add the first point to the end)
    sampled_points = np.vstack([sampled_points, sampled_points[0]])

    # 4. Split into X and Y and Save
    x_coords = sampled_points[:, 0]
    y_coords = sampled_points[:, 1]
    
    # Note: Images have (0,0) at top-left. We flip Y so it looks right in a graph.
    height = img.shape[0]
    y_coords = height - y_coords 

    save_points("points_x.txt", x_coords)
    save_points("points_y.txt", y_coords)
    
    print(f"Successfully processed image. Saved {len(sampled_points)} points.")

if __name__ == "__main__":
    main()
