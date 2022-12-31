fn main() {
    println!("{}", longest_palindrome(String::from("bb")));
}


pub fn longest_palindrome(s: String) -> String {
        let mut found: (usize, usize) = (0,1);
        let mut max_len = found.1 - found.0;
        let data = s.as_bytes();
        for index in 1..data.len() {
            let center = check_center(index, &s);
            let off_cenetr = check_with_start_and_end(index-1, index, &s);
            let len1 = center.1 - center.0;
            let len2 = off_cenetr.1 - off_cenetr.0;
            if len1 > max_len  {
                found = center;
                max_len = found.1 - found.0;
            }
            if len2 > max_len {
                found = off_cenetr;
                max_len = found.1 - found.0;
            }
        }
        return String::from_utf8(data[found.0..found.1].to_vec()).unwrap();
    } 

fn check_center(index: usize, s: &String) -> (usize, usize) {
    let iterator1 = index-1;
    let iterator2 = index+1;
    check_with_start_and_end(iterator1, iterator2, s)

}

fn check_with_start_and_end(start: usize, end:usize , s: &String) -> (usize, usize) {
    let bytes = s.as_bytes();
    let mut iterator1 : i32 = start as i32;
    let mut iterator2  = end as i32;
    while iterator1 >=0 && iterator2 < bytes.len() as i32 && bytes[iterator1 as usize] == bytes[iterator2 as usize] {
        iterator1-=1;
        iterator2+=1;
    }
    return ((iterator1+1) as usize, iterator2 as usize);
}


/*
public String longestPalindrome(String s) {
        String max = s.substring(0,1);
        for (int index = 1 ; index < s.length() ; index++){
            String str1 = checkCenter(index,s);
            String str2 = checkWithStartAndEnd(index-1,index,s);
            if (str1.length()> max.length()){
                max = str1;
            }
            if (str2.length()> max.length()){
                max = str2;
            }
        }
        return max;
    }
    
    public String checkCenter(int center, String s){
        int iterator1 = center-1;
        int iterator2 = center+1;
        while (iterator1>=0 && iterator2<s.length() && s.charAt(iterator1)==s.charAt(iterator2)){
            iterator1--;
            iterator2++;
        }
        return s.substring(iterator1+1,iterator2);
    }
    
    public String checkWithStartAndEnd(int start, int end, String s){
        while (start>=0 && end<s.length() && s.charAt(start)==s.charAt(end)){
            start--;
            end++;
        }
        return s.substring(start+1,end);
    }

*/