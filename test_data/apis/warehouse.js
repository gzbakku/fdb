const num_of_files = 10;
const no_of_items_per_file = 10;
const file_types = ['list','book','dict','config','map','index'];
const actions = ['add_item','','',''];

module.exports = {

  init:async ()=>{
    const path = io.cwd + '/data/warehouse.json'
    const items = make_files("list","add_item");
    await io.write(path,items);
  }

};

function make_files(type,func){
  let items = [];
  for(var i=0;i<num_of_files;i++){
    for(let item of make_file(func,type)){
      items.push(item);
    }
  }
  return items;
}

function make_file(func,file_type){
  const file_name = md5(uniqid()) + '-0_' + (no_of_items_per_file + 100);
  let items = [];
  for(var i=0;i<no_of_items_per_file;i++){
    items.push({
      type:func,
      data:{
        file_name:file_name,
        file_type:file_type,
        item_index:i.toString(),
        item_value:make_data()
      }
    });
  }
  return items;
}

function make_data(){
  return JSON.stringify({
    name:faker.name.findName(),
    address:faker.address.secondaryAddress(),
    company:faker.company.companyName(),
    jobTitle:faker.name.jobTitle(),
  });
}
