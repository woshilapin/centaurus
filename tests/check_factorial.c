# include <stdlib.h>
# include <stdio.h>
# include <check.h>
# include <limits.h>

# include "../src/factorial.h"

START_TEST(test_factorial_result)
{
	fail_unless( \
			factorial((unsigned long long int) 0) == (unsigned long long int) 1, \
			"Incorrect value for the factorial of 0 (%lu instead of 1).", \
			(unsigned long int) factorial((unsigned long long int) 0));
	fail_unless( \
			factorial((unsigned long long int) 1) == (unsigned long long int) 1, \
			"Incorrect value for the factorial of 1 (%lu instead of 1).", \
			(unsigned long int) factorial((unsigned long long int) 1));
	fail_unless( \
			factorial((unsigned long long int) 2) == (unsigned long long int) 2, \
			"Incorrect value for the factorial of 2 (%lu instead of 2).", \
			(unsigned long int) factorial((unsigned long long int) 2));
	fail_unless( \
			factorial((unsigned long long int) 5) == (unsigned long long int) 120, \
			"Incorrect value for the factorial of 5 (%lu instead of 120).", \
			(unsigned long int) factorial((unsigned long long int) 5));
}
END_TEST

START_TEST(test_factorial_overflow)
{
	unsigned long long int n = 1;
	unsigned long long int max_check = ULLONG_MAX / 2;
	for(n = 1; n < max_check; n=n*2)
	{
		fail_unless( \
				factorial(n) == factorial(n - 1) * n \
				|| factorial(n) == 0, \
				"Overflow value (n=%lu) should return 0 instead of %lu.", \
				(unsigned long int) n, \
				(unsigned long int) factorial(n));
	}
}
END_TEST

Suite * factorial_suite(void)
{
	Suite * s = suite_create("factorial");
	TCase * tc_core = tcase_create("core");
	tcase_add_test(tc_core, test_factorial_result);
	tcase_add_test(tc_core, test_factorial_overflow);
	suite_add_tcase(s, tc_core);

	return s;
}

int main(int argc, char ** argv)
{
	int number_failed = 0;
	Suite * s = factorial_suite();
	SRunner * sr = srunner_create(s);
	srunner_run_all(sr, CK_NORMAL);
	number_failed = srunner_ntests_failed(sr);
	srunner_free(sr);

	if(number_failed == 0)
	{
		return EXIT_SUCCESS;
	}
	else
	{
		return EXIT_FAILURE;
	}
}
