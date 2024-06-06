import { useMutation, useQuery } from '@apollo/client';
import { CreateNoteDocument, GetNotesDocument } from '@gql/graphql';
import './App.css';

function App() {
    const { data } = useQuery(GetNotesDocument);
    const [createNote] = useMutation(CreateNoteDocument);

    const handleClick = async () => {
        const r = await createNote({
            variables: {
                title: 'another note',
            },
        });
    };

    return (
        <div className="pt-6">
            <button onClick={handleClick}>Click Me</button>
        </div>
    );
}

export default App;
