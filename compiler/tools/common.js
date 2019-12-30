

function error(e){
  console.log("!!! " + e);
  return false;
}

function log(e){
  console.log(">>> " + e);
  return true;
}

module.exports= {
  error:error,
  log:log
};
