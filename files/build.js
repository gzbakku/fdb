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
          console.log("");
          console.log("");
          resolve();
        },
        onError:(e)=>{
          console.log(e);
          console.log("");
          console.log("");
          reject(e);
        },
      });

    });
  }
};
