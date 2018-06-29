let addon = require('../../native');

test('evaluate standard board', () => {
    expect(addon.testEvaluateBoard()).toBe(2388);
});