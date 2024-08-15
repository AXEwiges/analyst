use crate::reader::StreamingCsvReader;
use anyhow::Result;
use std::collections::{HashMap, HashSet};

type ItemSet = HashSet<String>;

pub fn analyze(reader: &mut StreamingCsvReader<'_>, min_support: f64) -> Result<()> {
    let headers = reader.headers().clone();
    let mut transactions: Vec<ItemSet> = Vec::new();
    let mut item_counts: HashMap<String, usize> = HashMap::new();

    // Read transactions and count items
    while let Some(result) = reader.next() {
        let record = result?;
        let transaction: ItemSet = record
            .iter()
            .enumerate()
            .filter(|(_, value)| !value.is_empty())
            .map(|(i, value)| format!("{}:{}", headers[i].to_string(), value))
            .collect();

        for item in &transaction {
            *item_counts.entry(item.to_string()).or_insert(0) += 1;
        }

        transactions.push(transaction);
    }

    let total_transactions = transactions.len();
    let min_support_count = (min_support * total_transactions as f64).ceil() as usize;

    // Filter frequent items
    let frequent_items: Vec<String> = item_counts
        .into_iter()
        .filter(|(_, count)| *count >= min_support_count)
        .map(|(item, _)| item)
        .collect();

    // Generate frequent itemsets
    let mut frequent_itemsets = vec![frequent_items];
    let mut k = 1;

    while !frequent_itemsets[k - 1].is_empty() {
        let candidates = generate_candidates(&frequent_itemsets[k - 1]);
        let frequent = find_frequent_itemsets(&candidates, &transactions, min_support_count);
        if !frequent.is_empty() {
            frequent_itemsets.push(frequent);
            k += 1;
        } else {
            break;
        }
    }

    // Print results
    println!("Frequent patterns (min support: {:.2}%):", min_support * 100.0);
    for (k, itemsets) in frequent_itemsets.iter().enumerate().skip(1) {
        println!("\n{}-item frequent patterns:", k);
        for itemset in itemsets {
            let support = count_support(itemset, &transactions) as f64 / total_transactions as f64;
            println!("  {} (support: {:.2}%)", itemset, support * 100.0);
        }
    }

    Ok(())
}

fn generate_candidates(prev_frequent: &[String]) -> Vec<String> {
    let mut candidates = Vec::new();
    for i in 0..prev_frequent.len() {
        for j in (i + 1)..prev_frequent.len() {
            let mut new_candidate: Vec<&str> = prev_frequent[i].split(',').collect();
            new_candidate.extend(prev_frequent[j].split(','));
            new_candidate.sort();
            new_candidate.dedup();
            if new_candidate.len() == prev_frequent[i].split(',').count() + 1 {
                candidates.push(new_candidate.join(","));
            }
        }
    }
    candidates
}

fn find_frequent_itemsets(candidates: &[String], transactions: &[ItemSet], min_support: usize) -> Vec<String> {
    candidates
        .iter()
        .filter(|itemset| count_support(itemset, transactions) >= min_support)
        .cloned()
        .collect()
}

fn count_support(itemset: &str, transactions: &[ItemSet]) -> usize {
    let items: HashSet<&str> = itemset.split(',').collect();
    transactions
        .iter()
        .filter(|transaction| items.iter().all(|item| transaction.contains(*item)))
        .count()
}