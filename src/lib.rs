use scrypto::prelude::*;

#[derive(ScryptoSbor, NonFungibleData)]
pub struct NFTData {
    name: String,
    description: String,
    icon_url: String,
    key_image_url: Url,
}

#[blueprint]
mod greatnftminter {

    struct GreatMinterNFT {
        sample_vault: Vault,
    }

    impl GreatMinterNFT {
        pub fn instantiate_and_mint_nft(
            name: String,
            description: String,
            icon_url: String,
        ) -> (Global<GreatMinterNFT>, Bucket) {
            // Clone strings to avoid ownership move issues
            let name_clone = name.clone();
            let description_clone = description.clone();
            let icon_url_clone = icon_url.clone();

            // Mint the NFT
            let mut my_bucket: Bucket = ResourceBuilder::new_ruid_non_fungible(OwnerRole::None)
                .metadata(metadata! {
                    init {
                        "name" => name_clone.as_str(), locked;
                        "description" => description_clone.as_str(), locked;
                        "icon_url" => Url::of(icon_url_clone.as_str()), locked;
                    }
                })
                .mint_initial_supply([NFTData {
                    name,
                    description,
                    icon_url,
                    key_image_url: Url::of(icon_url_clone.as_str()),
                }])
                .into();

            let nft_for_user = my_bucket.take(1);

            let component = Self {
                sample_vault: Vault::with_bucket(my_bucket),
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::None)
            .globalize();

            // Return the component and the NFT for the user
            (component, nft_for_user)
        }
    }
}
