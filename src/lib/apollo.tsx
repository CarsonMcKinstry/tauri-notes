import {
    ApolloProvider as AProvider,
    ApolloClient,
    ApolloLink,
    InMemoryCache,
    Observable,
} from '@apollo/client';
import { invoke } from '@tauri-apps/api';
import { print } from 'graphql';

const InvokeLink = new ApolloLink(
    ({ query, variables, operationName }) =>
        new Observable((observer) => {
            invoke('graphql', {
                query: print(query),
                variables,
                operationName,
            })
                .then(([data]: any) => {
                    observer.next({ data });
                    observer.complete();
                })
                .catch((err) => {
                    console.error(err);
                    if (Array.isArray(err)) {
                        for (const e of err) {
                            observer.error(e);
                        }
                    } else {
                        observer.error(err);
                    }
                });
        })
);

const client = new ApolloClient({
    cache: new InMemoryCache(),
    link: InvokeLink,
    defaultOptions: {
        query: {
            fetchPolicy: 'network-only',
        },
    },
});

import { ReactNode } from 'react';

type ApolloProviderProps = {
    children: ReactNode;
};

export const ApolloProvider = ({ children }: ApolloProviderProps) => {
    return <AProvider client={client}>{children}</AProvider>;
};
