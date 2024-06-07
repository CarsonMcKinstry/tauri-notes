import { useMutation, useQuery } from '@apollo/client';
import { CreateNoteDocument, GetNotesDocument } from '@gql/graphql';
import './App.css';

function App() {
    const { data } = useQuery(GetNotesDocument);
    const [createNote] = useMutation(CreateNoteDocument);
    const handleClick = async () => {
        await createNote({
            variables: {
                title: 'another note',
            },
        });
    };

    return (
        <div className="pt-6">
            <button onClick={handleClick}>Click Me</button>
            <code>
                <pre>{data && JSON.stringify(data, null, 4)}</pre>
            </code>
        </div>
    );
}

export default App;
