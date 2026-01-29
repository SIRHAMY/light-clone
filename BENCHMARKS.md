# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [arc_str](#arc_str)
    - [arc_large_struct](#arc_large_struct)
    - [five_arc_fields](#five_arc_fields)
    - [lc_vs_string_fields_by_len](#lc_vs_string_fields_by_len)
    - [nested_structs_3_levels_by_len](#nested_structs_3_levels_by_len)
    - [string_size_comparison_by_len](#string_size_comparison_by_len)
    - [arc_lc_vs_clone](#arc_lc_vs_clone)
    - [collection__clone_by_len](#collection__clone_by_len)
    - [collection__clone_then_mutate_by_len](#collection__clone_then_mutate_by_len)
    - [map__clone_by_len](#map__clone_by_len)
    - [map__clone_then_mutate_by_len](#map__clone_then_mutate_by_len)

## Benchmark Results

### arc_str

|        | `lc`                     | `clone`                          |
|:-------|:-------------------------|:-------------------------------- |
|        | `11.43 ns` (✅ **1.00x**) | `11.50 ns` (✅ **1.01x slower**)  |

### arc_large_struct

|        | `lc`                    | `clone`                         |
|:-------|:------------------------|:------------------------------- |
|        | `8.67 ns` (✅ **1.00x**) | `8.62 ns` (✅ **1.01x faster**)  |

### five_arc_fields

|        | `lc`                     | `clone`                          |
|:-------|:-------------------------|:-------------------------------- |
|        | `41.94 ns` (✅ **1.00x**) | `41.52 ns` (✅ **1.01x faster**)  |

### lc_vs_string_fields_by_len

|             | `arc_lc`                 | `string_clone`                     |
|:------------|:-------------------------|:---------------------------------- |
| **`10`**    | `41.66 ns` (✅ **1.00x**) | `49.17 ns` (❌ *1.18x slower*)      |
| **`100`**   | `42.88 ns` (✅ **1.00x**) | `52.26 ns` (❌ *1.22x slower*)      |
| **`1000`**  | `42.37 ns` (✅ **1.00x**) | `80.69 ns` (❌ *1.90x slower*)      |
| **`10000`** | `43.08 ns` (✅ **1.00x**) | `829.13 ns` (❌ *19.25x slower*)    |

### nested_structs_3_levels_by_len

|             | `arc_nested_lc`          | `string_nested_clone`              |
|:------------|:-------------------------|:---------------------------------- |
| **`10`**    | `23.93 ns` (✅ **1.00x**) | `33.33 ns` (❌ *1.39x slower*)      |
| **`100`**   | `23.39 ns` (✅ **1.00x**) | `37.14 ns` (❌ *1.59x slower*)      |
| **`1000`**  | `23.60 ns` (✅ **1.00x**) | `55.92 ns` (❌ *2.37x slower*)      |
| **`10000`** | `23.77 ns` (✅ **1.00x**) | `475.54 ns` (❌ *20.00x slower*)    |

### string_size_comparison_by_len

|             | `arc_str_lc`             | `string_clone`                   |
|:------------|:-------------------------|:-------------------------------- |
| **`10`**    | `11.77 ns` (✅ **1.00x**) | `12.71 ns` (✅ **1.08x slower**)  |
| **`100`**   | `11.66 ns` (✅ **1.00x**) | `13.72 ns` (❌ *1.18x slower*)    |
| **`1000`**  | `11.75 ns` (✅ **1.00x**) | `19.77 ns` (❌ *1.68x slower*)    |
| **`10000`** | `11.38 ns` (✅ **1.00x**) | `73.68 ns` (❌ *6.47x slower*)    |

### arc_lc_vs_clone

|        | `arc.light_clone()`          | `arc.clone()`                    |
|:-------|:-----------------------------|:-------------------------------- |
|        | `12.41 ns` (✅ **1.00x**)     | `12.33 ns` (✅ **1.01x faster**)  |

### collection__clone_by_len

|             | `std_vec`                 | `im_vector`                      | `imbl_vector`                    | `rpds_vector`                     |
|:------------|:--------------------------|:---------------------------------|:---------------------------------|:--------------------------------- |
| **`10`**    | `11.82 ns` (✅ **1.00x**)  | `9.56 ns` (✅ **1.24x faster**)   | `9.40 ns` (✅ **1.26x faster**)   | `1.05 ns` (🚀 **11.30x faster**)   |
| **`100`**   | `12.11 ns` (✅ **1.00x**)  | `40.45 ns` (❌ *3.34x slower*)    | `40.02 ns` (❌ *3.30x slower*)    | `1.05 ns` (🚀 **11.53x faster**)   |
| **`1000`**  | `46.60 ns` (✅ **1.00x**)  | `39.87 ns` (✅ **1.17x faster**)  | `39.76 ns` (✅ **1.17x faster**)  | `1.03 ns` (🚀 **45.15x faster**)   |
| **`10000`** | `613.29 ns` (✅ **1.00x**) | `39.72 ns` (🚀 **15.44x faster**) | `39.81 ns` (🚀 **15.40x faster**) | `1.04 ns` (🚀 **592.41x faster**)  |

### collection__clone_then_mutate_by_len

|             | `std_vec`                 | `im_vector`                     | `imbl_vector`                   | `rpds_vector`                     |
|:------------|:--------------------------|:--------------------------------|:--------------------------------|:--------------------------------- |
| **`10`**    | `26.16 ns` (✅ **1.00x**)  | `12.05 ns` (🚀 **2.17x faster**) | `12.10 ns` (🚀 **2.16x faster**) | `74.71 ns` (❌ *2.86x slower*)     |
| **`100`**   | `43.38 ns` (✅ **1.00x**)  | `80.13 ns` (❌ *1.85x slower*)   | `80.92 ns` (❌ *1.87x slower*)   | `86.67 ns` (❌ *2.00x slower*)     |
| **`1000`**  | `111.72 ns` (✅ **1.00x**) | `85.17 ns` (✅ **1.31x faster**) | `87.00 ns` (✅ **1.28x faster**) | `129.67 ns` (❌ *1.16x slower*)    |
| **`10000`** | `680.70 ns` (✅ **1.00x**) | `75.68 ns` (🚀 **8.99x faster**) | `75.06 ns` (🚀 **9.07x faster**) | `154.51 ns` (🚀 **4.41x faster**)  |

### map__clone_by_len

|             | `std_hashmap`             | `im_hashmap`                      | `imbl_hashmap`                    | `rpds_hashtriemap`                 |
|:------------|:--------------------------|:----------------------------------|:----------------------------------|:---------------------------------- |
| **`10`**    | `16.29 ns` (✅ **1.00x**)  | `14.65 ns` (✅ **1.11x faster**)   | `14.52 ns` (✅ **1.12x faster**)   | `1.40 ns` (🚀 **11.60x faster**)    |
| **`100`**   | `41.45 ns` (✅ **1.00x**)  | `15.04 ns` (🚀 **2.76x faster**)   | `14.36 ns` (🚀 **2.89x faster**)   | `1.37 ns` (🚀 **30.31x faster**)    |
| **`1000`**  | `131.12 ns` (✅ **1.00x**) | `14.49 ns` (🚀 **9.05x faster**)   | `14.85 ns` (🚀 **8.83x faster**)   | `1.36 ns` (🚀 **96.12x faster**)    |
| **`10000`** | `2.20 us` (✅ **1.00x**)   | `14.68 ns` (🚀 **149.92x faster**) | `14.38 ns` (🚀 **153.11x faster**) | `1.36 ns` (🚀 **1613.01x faster**)  |

### map__clone_then_mutate_by_len

|             | `std_hashmap`             | `im_hashmap`                     | `imbl_hashmap`                   | `rpds_hashtriemap`                |
|:------------|:--------------------------|:---------------------------------|:---------------------------------|:--------------------------------- |
| **`10`**    | `22.59 ns` (✅ **1.00x**)  | `122.24 ns` (❌ *5.41x slower*)   | `88.95 ns` (❌ *3.94x slower*)    | `94.24 ns` (❌ *4.17x slower*)     |
| **`100`**   | `44.58 ns` (✅ **1.00x**)  | `360.16 ns` (❌ *8.08x slower*)   | `295.66 ns` (❌ *6.63x slower*)   | `158.86 ns` (❌ *3.56x slower*)    |
| **`1000`**  | `131.03 ns` (✅ **1.00x**) | `635.98 ns` (❌ *4.85x slower*)   | `427.13 ns` (❌ *3.26x slower*)   | `183.83 ns` (❌ *1.40x slower*)    |
| **`10000`** | `2.29 us` (✅ **1.00x**)   | `703.85 ns` (🚀 **3.25x faster**) | `627.28 ns` (🚀 **3.65x faster**) | `278.65 ns` (🚀 **8.21x faster**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

