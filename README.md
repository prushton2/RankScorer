# RankScorer

A program to score how accurate a rank is. Looks at how close rach submission is to the position on the answer key, and gives more points for closer answers

## How to use
* input the answer in the answer key list
* imput the submission into the submitted answers list
* update the size of both lists
* run the program

## Score
* Score is a number <= 0
* Higher number is more accurate list

### How Score is Calculated
* For each item in the answer key, the same item is indexed in the submitted list
* The difference between the two indices of the item is the score removed
* the total score is the sum of all the removed scores
