# Analyst

Analyst is a command line tool that supports quick browsing of csv data, which can read csv dynamically in streaming mode and analyze it. It can support users to conveniently view missing values ​​of csv files, find frequent patterns of csv data, count the frequency of data in each column, find the maximum and minimum values ​​of a column, etc.

## Commands

* `show`: show rows, default 10 rows, max 100 rows
    * `analyst show file.csv --start {start} --end {end}`
* `missing-values`: show missing values
    * `analyst missing-values file.csv`
* `frequent-patterns`: show frequent patterns
    * `analyst frequent-patterns file.csv --min-support {ratio}`
* `column-stats`: show column statistics
    * `analyst column-stats file.csv --column {column}`
* `extrema`: show column extrema
    * `analyst extrema file.csv --column {column}`

## Example

Here is an example CSV file.
```csv
ID,Name,Age,Grade,Subject,Score,Attendance
1,Alice Smith,18,12,Math,95,98%
2,Bob Johnson,17,11,Physics,88,95%
3,Charlie Brown,16,10,Chemistry,78,92%
4,Diana Lee,,12,Biology,92,97%
5,Eva Martinez,18,12,Math,91,99%
6,Frank Wilson,17,11,,85,93%
7,Grace Taylor,16,10,Physics,89,96%
8,Henry Davis,18,12,Chemistry,,90%
9,Ivy Chen,17,11,Biology,94,98%
10,Jack Thompson,16,10,Math,82,
```

1. `analyst show test_data.csv`
```
+----+---------------+-----+-------+-----------+-------+------------+
| ID | Name          | Age | Grade | Subject   | Score | Attendance |
+----+---------------+-----+-------+-----------+-------+------------+
| 1  | Alice Smith   | 18  | 12    | Math      | 95    | 98%        |
+----+---------------+-----+-------+-----------+-------+------------+
| 2  | Bob Johnson   | 17  | 11    | Physics   | 88    | 95%        |
+----+---------------+-----+-------+-----------+-------+------------+
| 3  | Charlie Brown | 16  | 10    | Chemistry | 78    | 92%        |
+----+---------------+-----+-------+-----------+-------+------------+
| 4  | Diana Lee     |     | 12    | Biology   | 92    | 97%        |
+----+---------------+-----+-------+-----------+-------+------------+
| 5  | Eva Martinez  | 18  | 12    | Math      | 91    | 99%        |
+----+---------------+-----+-------+-----------+-------+------------+
| 6  | Frank Wilson  | 17  | 11    |           | 85    | 93%        |
+----+---------------+-----+-------+-----------+-------+------------+
| 7  | Grace Taylor  | 16  | 10    | Physics   | 89    | 96%        |
+----+---------------+-----+-------+-----------+-------+------------+
| 8  | Henry Davis   | 18  | 12    | Chemistry |       | 90%        |
+----+---------------+-----+-------+-----------+-------+------------+
| 9  | Ivy Chen      | 17  | 11    | Biology   | 94    | 98%        |
+----+---------------+-----+-------+-----------+-------+------------+
| 10 | Jack Thompson | 16  | 10    | Math      | 82    |            |
+----+---------------+-----+-------+-----------+-------+------------+
```

2. `analyst missing-values test_data.csv`
```
Total rows analyzed: 10
Missing value analysis:
Age: 1 missing values (10.00%)
Name: 0 missing values (0.00%)
Subject: 1 missing values (10.00%)
Score: 1 missing values (10.00%)
Attendance: 1 missing values (10.00%)
ID: 0 missing values (0.00%)
Grade: 0 missing values (0.00%)
```
3. `analyst column-stats test_data.csv --column Age`

```
Total rows analyzed: 10
Column statistics:

Column: Age
  18: 3 occurrences (30.00%)
  17: 3 occurrences (30.00%)
  16: 3 occurrences (30.00%)
  : 1 occurrences (10.00%)
```

4. `analyst extrema test_data.csv --column Score`

```
Extrema for column 'Score':
  Minimum value: 78
  Maximum value: 95
```

5. `analyst frequent-patterns test_data.csv --min-support 0.3`

```
Frequent patterns (min support: 30.00%):

1-item frequent patterns:
  Age:16,Grade:10 (support: 30.00%)
  Age:17,Grade:11 (support: 30.00%)
  Age:18,Grade:12 (support: 30.00%)
```