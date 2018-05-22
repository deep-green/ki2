import unittest

from src.Evaluator import Evaluator


class TestEvaluator(unittest.TestCase):
    def test_Evaluator(self):
        self.assertIsNotNone(Evaluator("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"))

    def test_getMove(self):
        self.assertEqual(Evaluator("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").getMove(),
                         "d2d4")


if __name__ == '__main__':
    unittest.main()
