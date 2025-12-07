#!/usr/bin/env python3
import os

HELLO = """fn main() {
    println!("Hello, world!");
}
"""

def main():
    confirmation= input("Are you sure? y/N\n")
    if confirmation != "y":
        return

    for root, dirs, files in os.walk("."):
        for name in files:
            # Replace contents of main.rs
            if name == "main.rs":
                path = os.path.join(root, name)
                print(f"Updating {path}")
                with open(path, "w", encoding="utf-8") as f:
                    f.write(HELLO)

            # Remove input.txt
            if name == "input.txt":
                path = os.path.join(root, name)
                print(f"Removing {path}")
                os.remove(path)
if __name__ == "__main__":
    main()
