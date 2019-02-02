#!/bin/bash

DOCS_DIR=pages_deploy/

cargo doc
rm -rf $DOCS_DIR
mv target/doc $DOCS_DIR

cat << EOF > $DOCS_DIR/index.html
<!DOCTYPE html>
<html>
<head>
<meta http-equiv="refresh" content="0; url=imgui_ext/index.html" />
</head>
<body>
    You are being redirected to the <a href="imgui_ext/index.html">documentation page</a>...
</body>
</html>
EOF

git -C $DOCS_DIR init && \
    git -C $DOCS_DIR remote add origin https://germangb:$TOKEN@github.com/germangb/imgui-ext.git && \
    git -C $DOCS_DIR checkout -b gh-pages && \
    git -C $DOCS_DIR add -A && \
    git -C $DOCS_DIR commit -m "Publish docs" && \
git -C $DOCS_DIR push origin gh-pages --force --quiet
