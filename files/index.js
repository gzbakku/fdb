

let nrc = require('node-run-cmd');

let run = "cargo run -- --secure=Om2lPq84vgIhsPEhWsh3LdRmNmI2MXpQ --signature=XW5L4OBPjuRcLhNUAvi40mOG3RdeJ6Pb --id=aXjD1ulK7VDP7yZRtmjVkbL6tMCUIhi5" +
" --session_id=XW5L4OBPjuRcLhNUAvi40mOG3RdeJ6Pb --session_signature=aXjD1ulK7VDP7yZRtmjVkbL6tMCUIhi5 --base_dir=d://workstation/expo/rust/fdb/composer/instance --port=8088 --composer=127.0.0.1";

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
