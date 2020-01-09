

let nrc = require('node-run-cmd');

let run = "cargo run -- -p=akku";

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
