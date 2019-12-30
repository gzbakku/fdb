const ask = require('inquirer');
const cmd = require('./cmd');
const run = require('./run');
const fs = require('./fs');
const common = require('./common');
const build = require('./build');
const nrc = require('node-run-cmd');

module.exports = {
  fs:fs,
  ask:ask,
  cmd:(data)=>{
    if(!data){
      return nrc;
    } else {
      return (r)=>{
        run;
      }
    }
  },
  build:build,
  common:common,
  cwd:()=>{
    return process.cwd();
  },
  os:()=>{
    let hold = process.platform;
    if(hold == 'win32'){
      return 'windows';
    }
    if(hold == 'linux'){
      return 'linux';
    }
  }
};
