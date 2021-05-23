const { loadBinding } = require('@node-rs/helper')

const { createApp: createNativeApp } = loadBinding(__dirname, 'hns', '@hnsjs/core')

/**
 * @param {number} port
 * @returns {void}
 */
exports.createApp = function createApp(port) {
  return new Promise((resolve, reject) => {
    createNativeApp(port, (err) => {
      if (err) {
        reject(err)
      } else {
        resolve()
      }
    }).catch(reject)
  })
}
