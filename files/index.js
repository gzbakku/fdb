

let nrc = require('node-run-cmd');

let run = "cargo run -- --secure=Om2lPq84vgIhsPEhWsh3LdRmNmI2MXpQ " +
"--actor_signature=XW5L4OBPjuRcLhNUAvi40mOG3RdeJ6Pb --actor_id=aXjD1ulK7VDP7yZRtmjVkbL6tMCUIhi5 " +
"--node_signature=XW5L4OBPjuRcLhNUAvi40mOG3RdeJ6Pb --node_id=aXjD1ulK7VDP7yZRtmjVkbL6tMCUIhi5 --node_port=7200 " +
"--composer_signature=XW5L4OBPjuRcLhNUAvi40mOG3RdeJ6Pb --composer_id=aXjD1ulK7VDP7yZRtmjVkbL6tMCUIhi5 --composer_ip=127.0.0.1 --composer_port=5200 " +
"--session_id=XW5L4OBPjuRcLhNUAvi40mOG3RdeJ6Pb --session_signature=aXjD1ulK7VDP7yZRtmjVkbL6tMCUIhi5 " +
"--base_dir=d://workstation/expo/rust/fdb/composer/instance " +
"--port=8088 --private_key=D://workstation/expo/rust/fdb/files/keys/key.pem --public_key=D://workstation/expo/rust/fdb/files/keys/cert.pem";

//total flags = 14

console.log(run);
console.log("");
console.log("");

nrc.run(run,{
  onData: (d)=>{
    console.log(d);
  },
  onDone:(d)=>{
    console.log(d);
  },
  onError:(d)=>{
    console.log(d);
  },
});
