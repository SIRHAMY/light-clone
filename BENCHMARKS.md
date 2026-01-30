# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [arc_str](#arc_str)
    - [arc_large_struct](#arc_large_struct)
    - [five_arc_fields](#five_arc_fields)
    - [lc_vs_string_fields_by_len](#lc_vs_string_fields_by_len)
    - [nested_structs_3_levels_by_len](#nested_structs_3_levels_by_len)
    - [deep_nested_by_depth](#deep_nested_by_depth)
    - [deep_nested_50_levels_by_size](#deep_nested_50_levels_by_size)
    - [string_size_comparison_by_len](#string_size_comparison_by_len)
    - [arc_lc_vs_clone](#arc_lc_vs_clone)
    - [string__clone_vs_mutate_by_len](#string__clone_vs_mutate_by_len)
    - [vec__clone_vs_mutate_by_len](#vec__clone_vs_mutate_by_len)
    - [struct__clone_vs_mutate_by_len](#struct__clone_vs_mutate_by_len)
    - [hashmap__clone_vs_mutate_by_len](#hashmap__clone_vs_mutate_by_len)
    - [collection__clone_by_len](#collection__clone_by_len)
    - [collection__clone_then_mutate_by_len](#collection__clone_then_mutate_by_len)
    - [map__clone_by_len](#map__clone_by_len)
    - [map__clone_then_mutate_by_len](#map__clone_then_mutate_by_len)
    - [persistent_vec__clone_vs_mutate_by_len](#persistent_vec__clone_vs_mutate_by_len)
    - [persistent_map__clone_vs_mutate_by_len](#persistent_map__clone_vs_mutate_by_len)

## Benchmark Results

### arc_str

|        | `lc`                     | `clone`                          |
|:-------|:-------------------------|:-------------------------------- |
|        | `11.31 ns` (✅ **1.00x**) | `11.30 ns` (✅ **1.00x faster**)  |

### arc_large_struct

|        | `lc`                    | `clone`                         |
|:-------|:------------------------|:------------------------------- |
|        | `8.51 ns` (✅ **1.00x**) | `8.53 ns` (✅ **1.00x slower**)  |

### five_arc_fields

|        | `lc`                     | `clone`                          |
|:-------|:-------------------------|:-------------------------------- |
|        | `41.16 ns` (✅ **1.00x**) | `40.97 ns` (✅ **1.00x faster**)  |

### lc_vs_string_fields_by_len

|             | `arc_lc`                 | `string_clone`                     |
|:------------|:-------------------------|:---------------------------------- |
| **`10`**    | `40.88 ns` (✅ **1.00x**) | `40.13 ns` (✅ **1.02x faster**)    |
| **`100`**   | `40.99 ns` (✅ **1.00x**) | `40.18 ns` (✅ **1.02x faster**)    |
| **`1000`**  | `40.95 ns` (✅ **1.00x**) | `61.77 ns` (❌ *1.51x slower*)      |
| **`10000`** | `41.10 ns` (✅ **1.00x**) | `800.00 ns` (❌ *19.47x slower*)    |

### nested_structs_3_levels_by_len

|             | `arc_nested_lc`          | `string_nested_clone`              |
|:------------|:-------------------------|:---------------------------------- |
| **`10`**    | `23.73 ns` (✅ **1.00x**) | `25.90 ns` (✅ **1.09x slower**)    |
| **`100`**   | `23.78 ns` (✅ **1.00x**) | `27.62 ns` (❌ *1.16x slower*)      |
| **`1000`**  | `23.65 ns` (✅ **1.00x**) | `43.52 ns` (❌ *1.84x slower*)      |
| **`10000`** | `23.76 ns` (✅ **1.00x**) | `465.49 ns` (❌ *19.59x slower*)    |

### deep_nested_by_depth

|           | `arc_lc`                 | `string_clone`                     |
|:----------|:-------------------------|:---------------------------------- |
| **`1`**   | `8.48 ns` (✅ **1.00x**)  | `19.62 ns` (❌ *2.31x slower*)      |
| **`5`**   | `14.98 ns` (✅ **1.00x**) | `81.09 ns` (❌ *5.41x slower*)      |
| **`10`**  | `15.01 ns` (✅ **1.00x**) | `234.52 ns` (❌ *15.62x slower*)    |
| **`25`**  | `14.94 ns` (✅ **1.00x**) | `842.23 ns` (❌ *56.37x slower*)    |
| **`50`**  | `14.96 ns` (✅ **1.00x**) | `1.91 us` (❌ *127.89x slower*)     |
| **`100`** | `14.98 ns` (✅ **1.00x**) | `4.18 us` (❌ *279.36x slower*)     |

### deep_nested_50_levels_by_size

|             | `arc_lc`                 | `string_clone`                     |
|:------------|:-------------------------|:---------------------------------- |
| **`10`**    | `15.06 ns` (✅ **1.00x**) | `1.13 us` (❌ *75.30x slower*)      |
| **`100`**   | `15.02 ns` (✅ **1.00x**) | `959.42 ns` (❌ *63.89x slower*)    |
| **`1000`**  | `14.99 ns` (✅ **1.00x**) | `1.61 us` (❌ *107.22x slower*)     |
| **`10000`** | `15.10 ns` (✅ **1.00x**) | `9.34 us` (❌ *618.65x slower*)     |

### string_size_comparison_by_len

|             | `arc_str_lc`             | `string_clone`                   |
|:------------|:-------------------------|:-------------------------------- |
| **`10`**    | `11.27 ns` (✅ **1.00x**) | `11.08 ns` (✅ **1.02x faster**)  |
| **`100`**   | `11.30 ns` (✅ **1.00x**) | `11.83 ns` (✅ **1.05x slower**)  |
| **`1000`**  | `11.28 ns` (✅ **1.00x**) | `17.99 ns` (❌ *1.59x slower*)    |
| **`10000`** | `11.25 ns` (✅ **1.00x**) | `82.86 ns` (❌ *7.37x slower*)    |

### arc_lc_vs_clone

|        | `arc.light_clone()`          | `arc.clone()`                    |
|:-------|:-----------------------------|:-------------------------------- |
|        | `11.81 ns` (✅ **1.00x**)     | `11.82 ns` (✅ **1.00x slower**)  |

### string__clone_vs_mutate_by_len

|             | `string_mutate`           | `arc_str_rebuild`                 |
|:------------|:--------------------------|:--------------------------------- |
| **`10`**    | `9.67 ns` (✅ **1.00x**)   | `23.54 ns` (❌ *2.44x slower*)     |
| **`100`**   | `22.49 ns` (✅ **1.00x**)  | `38.80 ns` (❌ *1.73x slower*)     |
| **`1000`**  | `50.78 ns` (✅ **1.00x**)  | `58.42 ns` (❌ *1.15x slower*)     |
| **`10000`** | `384.89 ns` (✅ **1.00x**) | `231.58 ns` (✅ **1.66x faster**)  |

### vec__clone_vs_mutate_by_len

|             | `vec_mutate`              | `vec_clone_then_mutate`           |
|:------------|:--------------------------|:--------------------------------- |
| **`10`**    | `19.64 ns` (✅ **1.00x**)  | `23.55 ns` (❌ *1.20x slower*)     |
| **`100`**   | `29.05 ns` (✅ **1.00x**)  | `37.46 ns` (❌ *1.29x slower*)     |
| **`1000`**  | `203.58 ns` (✅ **1.00x**) | `48.70 ns` (🚀 **4.18x faster**)   |
| **`10000`** | `1.56 us` (✅ **1.00x**)   | `653.45 ns` (🚀 **2.39x faster**)  |

### struct__clone_vs_mutate_by_len

|             | `struct_mutate`          | `lc_struct_clone`                |
|:------------|:-------------------------|:-------------------------------- |
| **`10`**    | `3.61 ns` (✅ **1.00x**)  | `23.66 ns` (❌ *6.56x slower*)    |
| **`100`**   | `4.49 ns` (✅ **1.00x**)  | `23.69 ns` (❌ *5.28x slower*)    |
| **`1000`**  | `3.88 ns` (✅ **1.00x**)  | `23.65 ns` (❌ *6.10x slower*)    |
| **`10000`** | `4.45 ns` (✅ **1.00x**)  | `23.61 ns` (❌ *5.31x slower*)    |

### hashmap__clone_vs_mutate_by_len

|             | `hashmap_mutate`          | `hashmap_clone_then_mutate`           |
|:------------|:--------------------------|:------------------------------------- |
| **`10`**    | `9.82 ns` (✅ **1.00x**)   | `15.14 ns` (❌ *1.54x slower*)         |
| **`100`**   | `11.54 ns` (✅ **1.00x**)  | `35.15 ns` (❌ *3.05x slower*)         |
| **`1000`**  | `19.81 ns` (✅ **1.00x**)  | `129.31 ns` (❌ *6.53x slower*)        |
| **`10000`** | `24.38 ns` (✅ **1.00x**)  | `2.55 us` (❌ *104.71x slower*)        |

### collection__clone_by_len

|             | `std_vec`                 | `im_vector`                      | `imbl_vector`                    | `rpds_vector`                     |
|:------------|:--------------------------|:---------------------------------|:---------------------------------|:--------------------------------- |
| **`10`**    | `10.07 ns` (✅ **1.00x**)  | `8.68 ns` (✅ **1.16x faster**)   | `8.68 ns` (✅ **1.16x faster**)   | `0.99 ns` (🚀 **10.21x faster**)   |
| **`100`**   | `10.07 ns` (✅ **1.00x**)  | `41.09 ns` (❌ *4.08x slower*)    | `41.06 ns` (❌ *4.08x slower*)    | `0.98 ns` (🚀 **10.22x faster**)   |
| **`1000`**  | `46.51 ns` (✅ **1.00x**)  | `40.96 ns` (✅ **1.14x faster**)  | `40.98 ns` (✅ **1.14x faster**)  | `0.99 ns` (🚀 **47.07x faster**)   |
| **`10000`** | `621.98 ns` (✅ **1.00x**) | `41.07 ns` (🚀 **15.14x faster**) | `40.94 ns` (🚀 **15.19x faster**) | `0.99 ns` (🚀 **628.25x faster**)  |

### collection__clone_then_mutate_by_len

|             | `std_vec`                 | `im_vector`                      | `imbl_vector`                    | `rpds_vector`                     |
|:------------|:--------------------------|:---------------------------------|:---------------------------------|:--------------------------------- |
| **`10`**    | `24.43 ns` (✅ **1.00x**)  | `12.26 ns` (🚀 **1.99x faster**)  | `12.24 ns` (🚀 **2.00x faster**)  | `57.91 ns` (❌ *2.37x slower*)     |
| **`100`**   | `40.37 ns` (✅ **1.00x**)  | `80.74 ns` (❌ *2.00x slower*)    | `81.17 ns` (❌ *2.01x slower*)    | `84.56 ns` (❌ *2.09x slower*)     |
| **`1000`**  | `107.20 ns` (✅ **1.00x**) | `83.07 ns` (✅ **1.29x faster**)  | `82.52 ns` (✅ **1.30x faster**)  | `112.70 ns` (✅ **1.05x slower**)  |
| **`10000`** | `1.28 us` (✅ **1.00x**)   | `73.62 ns` (🚀 **17.39x faster**) | `73.51 ns` (🚀 **17.41x faster**) | `145.31 ns` (🚀 **8.81x faster**)  |

### map__clone_by_len

|             | `std_hashmap`             | `im_hashmap`                      | `imbl_hashmap`                    | `rpds_hashtriemap`                 |
|:------------|:--------------------------|:----------------------------------|:----------------------------------|:---------------------------------- |
| **`10`**    | `14.33 ns` (✅ **1.00x**)  | `15.06 ns` (✅ **1.05x slower**)   | `15.07 ns` (✅ **1.05x slower**)   | `1.30 ns` (🚀 **11.01x faster**)    |
| **`100`**   | `31.71 ns` (✅ **1.00x**)  | `15.03 ns` (🚀 **2.11x faster**)   | `15.00 ns` (🚀 **2.11x faster**)   | `1.30 ns` (🚀 **24.48x faster**)    |
| **`1000`**  | `133.10 ns` (✅ **1.00x**) | `15.01 ns` (🚀 **8.87x faster**)   | `14.97 ns` (🚀 **8.89x faster**)   | `1.31 ns` (🚀 **101.94x faster**)   |
| **`10000`** | `2.22 us` (✅ **1.00x**)   | `14.98 ns` (🚀 **147.84x faster**) | `15.00 ns` (🚀 **147.65x faster**) | `1.29 ns` (🚀 **1711.14x faster**)  |

### map__clone_then_mutate_by_len

|             | `std_hashmap`             | `im_hashmap`                      | `imbl_hashmap`                   | `rpds_hashtriemap`                 |
|:------------|:--------------------------|:----------------------------------|:---------------------------------|:---------------------------------- |
| **`10`**    | `15.56 ns` (✅ **1.00x**)  | `123.86 ns` (❌ *7.96x slower*)    | `101.19 ns` (❌ *6.50x slower*)   | `75.00 ns` (❌ *4.82x slower*)      |
| **`100`**   | `34.79 ns` (✅ **1.00x**)  | `353.37 ns` (❌ *10.16x slower*)   | `330.98 ns` (❌ *9.51x slower*)   | `187.39 ns` (❌ *5.39x slower*)     |
| **`1000`**  | `137.66 ns` (✅ **1.00x**) | `533.81 ns` (❌ *3.88x slower*)    | `392.00 ns` (❌ *2.85x slower*)   | `127.71 ns` (✅ **1.08x faster**)   |
| **`10000`** | `2.21 us` (✅ **1.00x**)   | `709.10 ns` (🚀 **3.12x faster**)  | `635.83 ns` (🚀 **3.48x faster**) | `192.62 ns` (🚀 **11.48x faster**)  |

### persistent_vec__clone_vs_mutate_by_len

|             | `vec_mutate_only`          | `im_vector_clone_mutate`          | `imbl_vector_clone_mutate`          | `rpds_vector_clone_mutate`           |
|:------------|:---------------------------|:----------------------------------|:------------------------------------|:------------------------------------ |
| **`10`**    | `19.42 ns` (✅ **1.00x**)   | `11.64 ns` (✅ **1.67x faster**)   | `11.60 ns` (✅ **1.67x faster**)     | `54.83 ns` (❌ *2.82x slower*)        |
| **`100`**   | `31.37 ns` (✅ **1.00x**)   | `81.32 ns` (❌ *2.59x slower*)     | `80.73 ns` (❌ *2.57x slower*)       | `67.29 ns` (❌ *2.15x slower*)        |
| **`1000`**  | `202.79 ns` (✅ **1.00x**)  | `82.26 ns` (🚀 **2.47x faster**)   | `81.98 ns` (🚀 **2.47x faster**)     | `111.70 ns` (🚀 **1.82x faster**)     |
| **`10000`** | `1.50 us` (✅ **1.00x**)    | `73.30 ns` (🚀 **20.46x faster**)  | `72.94 ns` (🚀 **20.56x faster**)    | `135.13 ns` (🚀 **11.10x faster**)    |

### persistent_map__clone_vs_mutate_by_len

|             | `hashmap_mutate_only`          | `im_hashmap_clone_mutate`          | `imbl_hashmap_clone_mutate`          | `rpds_hashtriemap_clone_mutate`           |
|:------------|:-------------------------------|:-----------------------------------|:-------------------------------------|:----------------------------------------- |
| **`10`**    | `9.64 ns` (✅ **1.00x**)        | `182.51 ns` (❌ *18.92x slower*)    | `61.72 ns` (❌ *6.40x slower*)        | `71.31 ns` (❌ *7.39x slower*)             |
| **`100`**   | `12.90 ns` (✅ **1.00x**)       | `436.38 ns` (❌ *33.82x slower*)    | `268.23 ns` (❌ *20.79x slower*)      | `112.53 ns` (❌ *8.72x slower*)            |
| **`1000`**  | `16.72 ns` (✅ **1.00x**)       | `451.64 ns` (❌ *27.01x slower*)    | `427.13 ns` (❌ *25.54x slower*)      | `165.91 ns` (❌ *9.92x slower*)            |
| **`10000`** | `24.85 ns` (✅ **1.00x**)       | `709.69 ns` (❌ *28.56x slower*)    | `682.92 ns` (❌ *27.49x slower*)      | `192.83 ns` (❌ *7.76x slower*)            |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

