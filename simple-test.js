const { getDefinedAgent } = require('./index')

const agent = getDefinedAgent();

console.assert(agent === 'pnpm', 'Simple test failed')

console.info(`Simple test passed: ${agent}`)
