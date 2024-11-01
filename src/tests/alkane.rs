#[cfg(test)]
mod tests {
    use crate::tests::std::{alkanes_std_auth_token_build, alkanes_std_test_build};
    use alkanes_support::cellpack::Cellpack;
    use alkanes_support::id::AlkaneId;
    use anyhow::Result;
    use protorune::Protorune;

    use crate::tests::helpers as alkane_helpers;
    use crate::tests::std::alkanes_std_owned_token_build;
    use metashrew::clear;
    use wasm_bindgen_test::wasm_bindgen_test;

    use crate::message::AlkaneMessageContext;

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

        Protorune::index_block::<AlkaneMessageContext>(test_block, block_height as u64)?;
        Ok(())
    }

    #[wasm_bindgen_test]
    fn test_auth_token() -> Result<()> {
        clear();
        let block_height = 840_000;

        let test_cellpacks = [Cellpack {
            target: AlkaneId { block: 1, tx: 0 },
            inputs: vec![0, 1, 1000],
        }];

        let mut test_block = alkane_helpers::init_with_multiple_cellpacks(
            alkanes_std_owned_token_build::get_bytes(),
            test_cellpacks.to_vec(),
        );
        let auth_cellpack = Cellpack {
            target: AlkaneId {
                block: 3,
                tx: 0xffee,
            },
            inputs: vec![100],
        };
        let auth_block = alkane_helpers::init_with_multiple_cellpacks(
            alkanes_std_auth_token_build::get_bytes(),
            vec![auth_cellpack],
        );
        test_block.txdata = vec![auth_block.txdata[1].clone(), test_block.txdata[1].clone()];

        Protorune::index_block::<AlkaneMessageContext>(test_block, block_height as u64)?;

        Ok(())
    }

    #[wasm_bindgen_test]
    async fn test_base_std_functionality() -> Result<()> {
        clear();
        let input_cellpack = Cellpack {
            target: AlkaneId { block: 1, tx: 0 },
            inputs: vec![
                123456789123456789123456789u128,
                987654321987654321987654321u128,
            ],
        };

        let test_block = alkane_helpers::init_test_with_cellpack(input_cellpack);

        Protorune::index_block::<AlkaneMessageContext>(test_block.clone(), 840000 as u64)?;

        Ok(())
    }
}
