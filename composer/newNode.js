let nrc = require('node-run-cmd');

let run = "cargo run -- -p=akku --ip=127.0.0.1 --init --node --config=d://workstation/expo/rust/fdb/composer/instance/fdb.json -d=d://workstation/expo/rust/fdb/ --out=d://workstation/expo/rust/fdb/this_node_config.json";

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
