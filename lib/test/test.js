let addon = require('../../native');

test('evaluate standard board', () => {
    expect(addon.testEvaluateBoard()).toBe(0);
});

test('piece value of a pawn', () => {
   expect(addon.testGetPieceValueOfPawn()).toBe(11);
});

test('piece value of a knight', () => {
    expect(addon.testGetPieceValueOfKnight()).toBe(31);
});

test('piece value of a bishop', () => {
    expect(addon.testGetPieceValueOfBishop()).toBe(31);
});

test('piece value of a rook', () => {
    expect(addon.testGetPieceValueOfRook()).toBe(50);
});

test('piece value of a queen', () => {
    expect(addon.testGetPieceValueOfQueen()).toBe(90);
});

test('piece value of a king', () => {
    expect(addon.testGetPieceValueOfKing()).toBe(900);
});