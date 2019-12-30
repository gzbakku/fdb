

let nrc = require('node-run-cmd');

let cwd = process.cwd();

let public = cwd + '/keys/public.pem';
let private = cwd + '/keys/private.pem';

let run = "cargo run -- --public=" + public + ' --private=' + private;

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
