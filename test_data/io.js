const cwd =  process.cwd();
const fs = require('fs-extra');

module.exports = {

  cwd:cwd,

  write:(path,data)=>{
    return fs.writeJson(path, data)
    .then(()=>{
      return true;
    })
    .catch((e)=>{
      console.log(e);
      return false;
    });
  },

  read:(path)=>{
    return fs.readJson(path)
    .then((r) => {
      return r;
    })
    .catch((e)=>{
      console.log(e);
      return false;
    });
  }

};
