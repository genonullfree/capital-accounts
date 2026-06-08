# Capital Accounts

## What are capital accounts?

"the capital account, also known as the capital and financial account, records the net flow of investment into an economy." [Wikipedia](https://en.wikipedia.org/wiki/Capital_account)

This project allows a user to manage a single financial account that contains various sub-accounts within it. The software handles investment value changes as well as account deposits and withdrawals. It also handles removing accounts if necessary.

It also writes out to a log file for tracking account updates made over time.

## Usage

```bash
Usage: capitalaccounts [OPTIONS] <COMMAND>

Commands:
  deposit      Deposit value to an account
  withdraw     Withdraw value from an account
  list         List all accounts and their values
  investments  Update investment value
  new-user     Add a new user to an established account
  create       Create an entirely new account
  remove-user  Remove a user from an established account
  update       Automatically calculate the investment difference and apply it
  help         Print this message or the help of the given subcommand(s)

Options:
  -f, --file <FILE>  Filename to use [default: capital.act]
  -h, --help         Print help
```

### Example Investments Output

```bash
$ capitalaccounts -f investments.act investments -v -5.00
Loaded: investments.act
Investments: $-5.00
---
Account Totals:
  Total Value: $330.83

Name: geno
  Percent: 32.26%
  Value: $106.72
Name: mrs.geno
  Percent: 64.52%
  Value: $213.44
Name: lil.geno
  Percent: 3.23%
  Value: $10.67
---
Saved: investments.act
Action logged to investments.act.log
```

### Example Remove User Output
```bash
$ capitalaccounts -f investments.act remove-user -u mrs.geno withdraw
Loaded: investments.act
---
Account Totals:
  Total Value: $330.83

Name: geno
  Percent: 32.26%
  Value: $106.72
Name: mrs.geno
  Percent: 64.52%
  Value: $213.44
Name: lil.geno
  Percent: 3.23%
  Value: $10.67
---
User mrs.geno removed and $213.44 value withdrawn
---
Account Totals:
  Total Value: $117.39

Name: geno
  Percent: 90.91%
  Value: $106.72
Name: lil.geno
  Percent: 9.09%
  Value: $10.67
---
Saved: investments.act
Action logged to investments.act.log

$ capitalaccounts -f investments.act remove-user -u lil.geno disburse
Loaded: investments.act
---
Account Totals:
  Total Value: $117.39

Name: geno
  Percent: 90.91%
  Value: $106.72
Name: lil.geno
  Percent: 9.09%
  Value: $10.67
---
User lil.geno removed and $10.67 value disbursed
Investments: $10.67
---
Account Totals:
  Total Value: $117.39

Name: geno
  Percent: 100.00%
  Value: $117.39
---
Saved: investments.act
Action logged to investments.act.log
```

### Example Ledger Log
```bash
$ cat investments.act.log
2026-06-08 16:59:19.158001291 -04:00: New account created, geno deposited $100.00
2026-06-08 16:59:43.503215817 -04:00: mrs.geno deposited $200.00
2026-06-08 17:00:07.539339903 -04:00: lil.geno deposited $10.00
2026-06-08 17:01:00.282504305 -04:00: Investments gain/loss $22.50
2026-06-08 17:01:38.543515367 -04:00: Investments gain/loss $3.33
2026-06-08 17:01:48.481175999 -04:00: Investments gain/loss $-5.00
2026-06-08 17:02:31.705940092 -04:00: User mrs.geno removed, $213.44 withdrawn
2026-06-08 17:06:13.850399807 -04:00: User lil.geno removed, $10.67 disbursed
```
