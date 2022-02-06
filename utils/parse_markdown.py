import sys
import re
import markdown
from pathlib import Path

def main():
    if len(sys.argv) != 2:
        print("Wrong number of arguments")
        return
    
    with open(sys.argv[1], "r") as f:
        data = f.read()

    data = re.sub("\!\[\[(.*)\]\]", "![\g<1>](image/posts/\g<1>)", data)

    html = markdown.markdown(data)

    out_path = "static/posts/" + Path(sys.argv[1]).stem + ".html"
    with open(out_path, "w") as f:
        f.write(html)

    print(f"Sucessfully written HTML output to {out_path}")


if __name__ == "__main__":
    main()
