module.exports = {
  init:(path)=>{
    return new Promise((resolve,reject)=>{

      let run = "cargo build --release --manifest-path " + path;

      console.log(run);
      console.log("");
      console.log("");

      engine.cmd().run(run,{
        onData: (d)=>{
          console.log(d);
        },
        onDone:(d)=>{
          engine.common.log("command executed successfully");
          console.log(d);
          console.log("");
          console.log("");
          resolve();
        },
        // onError:(e)=>{
        //   engine.common.error("command execution failed");
        //   console.log(e);
        //   console.log("");
        //   console.log("");
        //   reject(e);
        // },
      });

    });
  }
};
