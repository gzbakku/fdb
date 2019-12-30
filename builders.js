const cert = require('./cert/build.js');
const composer = require('./composer/build.js');
const files = require('./files/build.js');

module.exports = {
  cert:cert,
  composer:composer,
  files:files
};
