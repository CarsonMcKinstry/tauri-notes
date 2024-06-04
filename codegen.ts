import type { CodegenConfig } from '@graphql-codegen/cli';

const config: CodegenConfig = {
    overwrite: true,
    schema: './schema.graphql',
    documents: ['src/**/*.{ts,tsx}', 'src/**/*.graphql'],
    generates: {
        './src/generated/graphql/': {
            preset: 'client',
            presetConfig: {
                gqlTagName: 'gql',
            },
        },
    },
    ignoreNoDocuments: true,
};

export default config;
