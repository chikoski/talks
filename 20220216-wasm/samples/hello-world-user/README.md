# hello-world-user

This folder contains a sample project using Wasm file created with wasm-pack. 

# Hwo to build.

This project relies on an artifact of "hello-world" project, which is placed in the parent folder. Please visit the folder and create a npm package. Details are described in the README.md in the folder.

The project is build with the following steps:

~~~sh
% npm i
% cp node_modules/hello-world/hello_world_bg.wasm ./public
% npm run build
~~~

# License

All files in this folder are distributed under MIT License.