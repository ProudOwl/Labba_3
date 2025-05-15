#include <iostream>
#include <vector>
using namespace std;

int main() {
    int n, m;
    long sum;
    cin >> n >> m;
    vector<int> nums(n);
    vector<long> dp(n + 1, -1);
    for (auto& i : nums) {
        cin >> i;
    }

    vector<long> prefix_sum(n + 1, 0);
    for (int i = 1; i < n; i++) {
        prefix_sum[i + 1] = prefix_sum[i] + nums[i];
    }

    dp[n] = 0;
    for (int i = n -1; i >= 0; --i) {
        int max_numbers = min(m, n - 1);
        for (int j = 1; j <= max_numbers; ++j) {
            sum = prefix_sum[i + j] - prefix_sum[i];
            dp[i] = max(dp[i], sum - dp[i + j]);
        }
    }

    if(dp[0] > 0) {
        cout << "1" << endl;
    } else {
        cout << "0" << endl;
    }
    return 0;
}