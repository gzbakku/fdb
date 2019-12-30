const fs = require('fs-extra');

module.exports = {

  copy:(from,to)=>{

    return fs.copy(from, to)
    .then(()=>{
      return true;
    })
    .catch((e)=>{
      return engine.common.error("failed to copy file e : " + e);
    })

  }

};
