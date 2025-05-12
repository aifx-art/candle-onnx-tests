  ```
  cargo run -r -- --image face-512.png --output facex.png --model 4xNomosWebPhoto_esrgan_fp32_opset17.onnx
  ```

  
  The system library `alsa` required by crate `alsa-sys` was not found.
  ```
  sudo apt-get install -y libasound2-dev
  ```


  got to load on a small tile and it looks like op_type resize is not supported
  ```
  Error: unsupported op_type Resize for op NodeProto { input: ["/Add_92_output_0", "", "/Constant_786_output_0"], output: ["/Resize_output_0"], name: "/Resize", op_type: "Resize", domain: "", attribute: [AttributeProto { name: "coordinate_transformation_mode", ref_attr_name: "", doc_string: "", r#type: String, f: 0.0, i: 0, s: [97, 115, 121, 109, 109, 101, 116, 114, 105, 99], t: None, g: None, sparse_tensor: None, tp: None, floats: [], ints: [], strings: [], tensors: [], graphs: [], sparse_tensors: [], type_protos: [] }, AttributeProto { name: "cubic_coeff_a", ref_attr_name: "", doc_string: "", r#type: Float, f: -0.75, i: 0, s: [], t: None, g: None, sparse_tensor: None, tp: None, floats: [], ints: [], strings: [], tensors: [], graphs: [], sparse_tensors: [], type_protos: [] }, AttributeProto { name: "mode", ref_attr_name: "", doc_string: "", r#type: String, f: 0.0, i: 0, s: [110, 101, 97, 114, 101, 115, 116], t: None, g: None, sparse_tensor: None, tp: None, floats: [], ints: [], strings: [], tensors: [], graphs: [], sparse_tensors: [], type_protos: [] }, AttributeProto { name: "nearest_mode", ref_attr_name: "", doc_string: "", r#type: String, f: 0.0, i: 0, s: [102, 108, 111, 111, 114], t: None, g: None, sparse_tensor: None, tp: None, floats: [], ints: [], strings: [], tensors: [], graphs: [], sparse_tensors: [], type_protos: [] }], doc_string: "" }
  ```

op_type resize is implemented now
```
Error: unsupported op_type DepthToSpace for op NodeProto { input: ["/model/body.66/Conv_output_0"], output: ["/model/upsampler/DepthToSpace_output_0"], name: "/model/upsampler/DepthToSpace", op_type: "DepthToSpace", domain: "", attribute: [AttributeProto { name: "blocksize", ref_attr_name: "", doc_string: "", r#type: Int, f: 0.0, i: 4, s: [], t: None, g: None, sparse_tensor: None, tp: None, floats: [], ints: [], strings: [], tensors: [], graphs: [], sparse_tensors: [], type_protos: [] }, AttributeProto { name: "mode", ref_attr_name: "", doc_string: "", r#type: String, f: 0.0, i: 0, s: [67, 82, 68], t: None, g: None, sparse_tensor: None, tp: None, floats: [], ints: [], strings: [], tensors: [], graphs: [], sparse_tensors: [], type_protos: [] }], doc_string: "" }
```





