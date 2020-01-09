const engine = require('./compiler/tools/index');
const cwd = engine.cwd();

global.engine = engine;

engine.ask
  .prompt([
    {
      type: 'checkbox',
      name:'actors',
      message:'please select the actors you wanna build',
      choices:[
        {name:'all',default:false},
        {name:'files',default:true},
        {name:'cert',default:false},
        {name:'composer',default:false}
      ],
      default:'files'
    }
  ])
  .then(async (answers)=>{

    //loop and check if all actor are to be compiled

    console.log(answers);

    let do_all = false;

    if(answers.actors.length == 0){
      return engine.common.error("please select atleast one");
    }

    let collect = [];
    for(let ans of answers.actors){
      if(ans == "all"){
        do_all = true;
      }
    }

    if(do_all){
      answers.actors = ['composer','files','cert'];
    }

    let all_built = true;
    for(let ans of answers.actors){
      let do_this = await build_actor(ans);
      if(!do_this){
        all_built = false
        break;
      }
    }

    if(!all_built){
      return engine.common.error("failed-build_actors");
    }

    return engine.common.log("compilation complete");

  });

async function build_actor(type){

  let ext = '';
  if(engine.os() == 'windows'){
    ext = '.exe';
  }

  let cargo_path = cwd + '/' + type + '/Cargo.toml';
  let actor_path = cwd + '/' + type + '/target/release/' + type + ext;
  let composer_debug_dir_path = cwd + '/composer/target/debug/' + type + ext;
  let build_path = cwd + '/build/' + type + ext;

  //console.log(cargo_path);

  let build = await engine.build.init(cargo_path)
  .then(()=>{
    return true;
  })
  .catch((e)=>{
    console.log(e);
    return false;
  });

  if(!build){
    return engine.common.error("failed-build_this_actor-" + type);
  }

  if(type !== 'composer'){
    let copy_to_compoer = await engine.fs.copy(actor_path,composer_debug_dir_path);
    if(!copy_to_compoer){
      return engine.common.error("failed-copy_to_compoer-" + type);
    }
  }

  let copy_to_build = await engine.fs.copy(actor_path,build_path);
  if(!copy_to_build){
    return engine.common.error("failed-copy_to_compoer-" + type);
  }

  return true;

}
