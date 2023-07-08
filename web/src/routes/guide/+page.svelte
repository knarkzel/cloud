<script lang="ts">
  import { CodeBlock } from "@skeletonlabs/skeleton";

  const lib = `
[lib]
crate-type = ["cdylib"]
  `;

  const main = `
use cloud_lib::{cloud, Result};
use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
struct Input {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct Output {
    valid: bool,
    message: String,
}

#[cloud]
fn main(input: Input) -> Result<Output> {
    let output = if input.username == input.password {
        Output {
            valid: true,
            message: "Username and password are equal!".to_owned(),
        }
    } else {
        Output {
            valid: false,
            message: "Username and password aren't equal!".to_owned(),
        }
    };
    Ok(output)
}
  `;
</script>

<h1 class="h1">Create WASM file</h1>

<p class="mt-8">
  Create a new project with <code>cargo new PROJECT --lib</code>, then add following
  to <code>Cargo.toml</code> so that it is a <code>cdylib</code>:
</p>

<div class="mt-8">
  <CodeBlock background="bg-slate-800" language="toml" code={lib} />
</div>

<p class="mt-8">Next, run <code>cargo add cloud-lib serde --features serde/derive</code>, then modify src/lib.rs:</p>

<div class="mt-8">
  <CodeBlock background="bg-slate-800" language="rust" code={main} />
</div>

<p class="mt-8">Now you can build this with <code>cargo build --release --target=wasm32-wasi</code>, then publish it to Cloud under <a class="anchor" href="/upload">Upload</a>.</p>
