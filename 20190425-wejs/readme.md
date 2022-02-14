# JS Binary handling 101

## TL;DR; 

* ArrayBuffer / View objects
* Slice

## ArrayBuffer

* 

## Load binary data in browser

~~~javascript
const res = await fetch("some-binary.data");
const buffer = await res.arrayBuffer();
~~~

## Load binary with File API

~~~javascript
const reader = new FileReader();
reader.addEventListener("load", e => {
  const buffer = e.result;
});

document.querySelector("input[type=file]")
  .addEventListener("change", e = >{
     reader.readAsArrayBuffer(e.files[0]);
  });
~~~

## Load binary data in 
  