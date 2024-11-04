#[cfg(test)]
mod tests {
    use crate::tests::std::{alkanes_std_auth_token_build, alkanes_std_test_build};
    use alkanes_support::cellpack::Cellpack;
    use alkanes_support::id::AlkaneId;
    use anyhow::Result;
    use metashrew_support::{index_pointer::KeyValuePointer, utils::format_key};
    use protorune::Protorune;

    use crate::index_block;
    use crate::tests::helpers as alkane_helpers;
    use crate::tests::std::alkanes_std_owned_token_build;
    use metashrew::{clear, get_cache, index_pointer::IndexPointer, println, stdio::stdout};
    use std::fmt::Write;
    use wasm_bindgen_test::wasm_bindgen_test;

    use crate::message::AlkaneMessageContext;
    pub fn print_cache() {
        let cache = get_cache();

        for (key, value) in cache.iter() {
            let formatted_key = format_key(key);
            let formatted_value = format_key(value);

            println!("{}: {}", formatted_key, formatted_value.len());
        }
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
    fn test_auth_token() -> Result<()> {
        clear();
        let block_height = 840_000;

        let test_cellpack = Cellpack {
            target: AlkaneId { block: 1, tx: 0 },
            inputs: vec![
                0,    /* opcode (to init new auth token) */
                1,    /* auth_token units */
                1000, /* owned_token token_units */
            ],
        };

        let auth_cellpack = Cellpack {
            target: AlkaneId {
                block: 3,
                tx: 0xffee,
            },
            inputs: vec![100],
        };
        let test_block = alkane_helpers::init_with_multiple_cellpacks_with_tx(
            [
                alkanes_std_auth_token_build::get_bytes(),
                alkanes_std_owned_token_build::get_bytes(),
            ]
            .into(),
            [auth_cellpack, test_cellpack].into(),
        );

        index_block(&test_block, block_height)?;

        println!("block indexed");
        let auth_token_id_factory = AlkaneId {
            block: 4,
            tx: 0xffee,
        };

        let auth_token_id_deployment = AlkaneId { block: 2, tx: 1 };
        let owned_token_id = AlkaneId { block: 2, tx: 0 };

        // assert_eq!(
        //     IndexPointer::from_keyword("/alkanes/")
        //         .select(&owned_token_id.into())
        //         .get(),
        //     alkanes_std_owned_token_build::get_bytes().into()
        // );
        assert_eq!(
            IndexPointer::from_keyword("/alkanes/")
                .select(&auth_token_id_factory.into())
                .get(),
            alkanes_std_auth_token_build::get_bytes().into()
        );
        // assert_eq!(
        //     IndexPointer::from_keyword("/alkanes/")
        //         .select(&auth_token_id_deployment.into())
        //         .get(),
        //     alkanes_std_auth_token_build::get_bytes().into()
        // );

        Ok(())
    }

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
                .get(),
            alkanes_std_test_build::get_bytes().into()
        );

        Ok(())
    }
}
