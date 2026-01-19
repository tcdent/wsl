"""
Read Evaluation Module

Tests how well LLMs can respond to Worldview-encoded beliefs.
Given worldview content as context, evaluates if the model's responses
align with the encoded beliefs.
"""

from .evaluator import (
    EvalScore,
    EvalResult,
    EvalSummary,
    evaluate_response,
    summarize_results,
)
from .runner import (
    EvalRunner,
    generate_report,
    generate_json_results,
)
from .test_cases import (
    Category,
    Difficulty,
    ExpectedBehavior,
    TestCase,
    ALL_TEST_CASES,
    get_cases_by_difficulty,
    get_cases_by_category,
    get_case_by_id,
)

__all__ = [
    # Evaluator
    "EvalScore",
    "EvalResult",
    "EvalSummary",
    "evaluate_response",
    "summarize_results",
    # Runner
    "EvalRunner",
    "generate_report",
    "generate_json_results",
    # Test cases
    "Category",
    "Difficulty",
    "ExpectedBehavior",
    "TestCase",
    "ALL_TEST_CASES",
    "get_cases_by_difficulty",
    "get_cases_by_category",
    "get_case_by_id",
]
