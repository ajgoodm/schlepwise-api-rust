set -e

PS3="Please select environment (type the number): "
options=("local")
select option in "${options[@]}"; do
    case $option in
        "local")
            IA_ENV=$option; break;;
        *)
            echo "Invalid option. Try again."; continue;;
    esac
done

yarn dotenvi -s $option
