import { useQuery } from '@apollo/client';
import { MyAppDocument } from '@gql/graphql';
import './App.css';

function App() {
    const { data } = useQuery(MyAppDocument);
    console.log(data?.apiVersion);
    return (
        <div>
            <h1>API Version: {data?.apiVersion}</h1>
        </div>
    );
}

export default App;
