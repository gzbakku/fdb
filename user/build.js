module.exports = {
  init:()=>{
    return new Promise((resolve,reject)=>{

      let run = "cargo build --release";

      console.log(run);
      console.log("");
      console.log("");

      engine.cmd.run(run,{
        onData: (d)=>{
          console.log(d);
        },
        onDone:(d)=>{
          console.log(d);
          resolve();
        },
        onError:(d)=>{
          console.log(d);
          reject();
        },
      });

    });
  }
};
