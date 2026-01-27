# Fluxlock Phase 9 Analysis

This repository contains analysis for **Phase 9** of the Fluxlock simulation.

## Overview

Phase 9 involves tracking node metrics including:

- `reputation`
- `entropy`
- `weighted_consensus`
- `stake`

The analysis scripts generate graphs and summary CSVs for all nodes.

## Files

- `node*_log.csv`: Node log data files
- `analyze_phase9.py`: Python script to generate plots and summary CSV
- `phase9_*.png`: Generated graphs for each metric
- `phase9_summary.csv`: Consolidated summary per tick

## How to Run

Make sure you have Python 3 and the required packages installed:

```bash
pip install pandas matplotlib
