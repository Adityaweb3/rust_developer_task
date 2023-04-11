#include <iostream>
#include <vector>
#include <algorithm>

int kth_smallest(std::vector<int>& arr, int k) {
    std::sort(arr.begin(), arr.end());
    return arr[k - 1];
}

int main() {
    std::vector<int> arr = {5, 2, 1, 3, 6, 4};
    int k = 3;
    int kth = kth_smallest(arr, k);
    std::cout << "The " << k << "th smallest element is: " << kth << std::endl;
    return 0;
}