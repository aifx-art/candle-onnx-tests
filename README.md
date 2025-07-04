  ```
  cargo run -r -- --image 32.png --output upres.png --model 4xNomosWebPhoto_esrgan_fp32_opset17.onnx
  ```

  
  The system library `alsa` required by crate `alsa-sys` was not found.
  ```
  sudo apt-get install -y libasound2-dev
  ```


## flux kontext input shape
Input name: hidden_states
Input name: encoder_hidden_states
Input name: pooled_projections
Input name: timestep
Input name: img_ids
Input name: txt_ids
Input name: guidance
Input name: base_model.double_blocks.0.img_mod.lin.weight_f4