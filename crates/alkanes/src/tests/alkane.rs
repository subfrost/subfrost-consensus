#[cfg(test)]
mod tests {
    use crate::tests::std::alkanes_std_test_build;
    use alkanes_support::cellpack::Cellpack;
    use alkanes_support::id::AlkaneId;
    use anyhow::Result;
    use metashrew_support::index_pointer::KeyValuePointer;

    use crate::index_block;
    use crate::tests::helpers as alkane_helpers;
    use alkanes_support::gz::{compress, decompress};
    #[allow(unused_imports)]
    use metashrew::{
        clear,
        index_pointer::IndexPointer,
        println,
        stdio::{stdout, Write},
    };
    use wasm_bindgen_test::wasm_bindgen_test;

    #[wasm_bindgen_test]
    pub fn test_compression() -> Result<()> {
        let buffer = alkanes_std_test_build::get_bytes();
        let compressed = compress(buffer.clone())?;
        assert_eq!(decompress(compressed)?, buffer.clone());
        Ok(())
    }

    #[wasm_bindgen_test]
    fn test_extcall() -> Result<()> {
        clear();
        let block_height = 840_000;

        let test_cellpacks = [
            //create alkane
            Cellpack {
                target: AlkaneId { block: 1, tx: 0 },
                inputs: vec![1],
            },
            /*
            //create second alkane
            Cellpack {
                target: AlkaneId { block: 1, tx: 0 },
                inputs: vec![0],
            },
            //target second alkane to be called with custom opcode
            Cellpack {
                target: AlkaneId { block: 2, tx: 0 },
                inputs: vec![1, 1],
            },
            */
        ];

        let test_block = alkane_helpers::init_with_multiple_cellpacks(
            alkanes_std_test_build::get_bytes(),
            test_cellpacks.to_vec(),
        );

        index_block(&test_block, block_height as u32)?;
        Ok(())
    }

    /*
        #[wasm_bindgen_test]
        fn test_benchmark() -> Result<()> {
            clear();
            let block_height = 840_000;

            let test_cellpacks = [
                //create alkane
                Cellpack {
                    target: AlkaneId { block: 1, tx: 0 },
                    inputs: vec![78],
                },
                /*
                //create second alkane
                Cellpack {
                    target: AlkaneId { block: 1, tx: 0 },
                    inputs: vec![0],
                },
                //target second alkane to be called with custom opcode
                Cellpack {
                    target: AlkaneId { block: 2, tx: 0 },
                    inputs: vec![1, 1],
                },
                */
            ];

            let start = metashrew::imports::__now();
            let test_block = alkane_helpers::init_with_multiple_cellpacks(
                alkanes_std_test_build::get_bytes(),
                test_cellpacks.to_vec(),
            );

            index_block(&test_block, block_height as u32)?;
            println!("time: {}ms", metashrew::imports::__now() - start);
            Ok(())
        }
    */

    #[wasm_bindgen_test]
    async fn test_base_std_functionality() -> Result<()> {
        clear();
        let test_target = AlkaneId { block: 1, tx: 0 };
        let test_stored_target = AlkaneId { block: 2, tx: 0 };
        let input_cellpack = Cellpack {
            target: test_target,
            inputs: vec![
                123456789123456789123456789u128,
                987654321987654321987654321u128,
            ],
        };

        let test_block = alkane_helpers::init_test_with_cellpack(input_cellpack);

        index_block(&test_block, 840000 as u32)?;
        assert_eq!(
            IndexPointer::from_keyword("/alkanes/")
                .select(&test_stored_target.into())
                .get()
                .as_ref()
                .clone(),
            compress(alkanes_std_test_build::get_bytes().into())?
        );

        Ok(())
    }
}
