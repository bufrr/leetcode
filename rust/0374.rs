/** 
 * Forward declaration of guess API.
 * @param  num   your guess
 * @return 	     -1 if num is higher than the picked number
 *			      1 if num is lower than the picked number
 *               otherwise return 0
 * unsafe fn guess(num: i32) -> i32 {}
 */

impl Solution {
    unsafe fn guessNumber(n: i32) -> i32 {
        let mut low = 1;
        let mut high = n;

        while (low <= high) {
            let mut mid = low + (high - low) / 2;
            let res = guess(mid);
            if res == 0 {
                return mid
            } else if res < 0 {
                high = mid - 1
            } else {
                low = mid + 1
            }
        }        
        return -1;
    }
}
