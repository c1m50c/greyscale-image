from typing import List
import cv2
import sys


def main(args: List[str]):
    if len(args) == 0:
        raise ValueError("Missing 'FilePath' Argument.")
    
    image_path: str = args.pop(0)
    image = cv2.imread(image_path)
    
    gs_image = cv2.cvtColor(image, cv2.COLOR_BGR2GRAY)
    cv2.imwrite("output.jpg", gs_image)


if __name__ == "__main__":
    main(sys.argv[1:])